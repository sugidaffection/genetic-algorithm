extern crate rand;
use rand::Rng;

pub struct DNA{
	fitness: i32,
	genes: Vec<u8>,
	target: String
}

impl DNA {

	fn new(target: String) -> DNA {
		let mut genes = vec![];
		for _ in 0..target.len(){
			genes.push(rand::thread_rng().gen_range(32, 126));
		}

		DNA {
			fitness: 0,
			genes: genes,
			target: target
		}
	}

	fn copy(&self) -> DNA{
		DNA{
			fitness: 0,
			genes: self.genes.clone(),
			target: self.target.to_owned()
		}
	}

	fn calculate_fitness(&mut self) {
		self.fitness = 0;
		for (x,y) in self.target.chars().map(|x| x as u8).zip(&self.genes){
			if x == *y {
				self.fitness += 1;
			}
		}
	}

	fn crossover(&self, partner: &DNA) -> DNA {
		let mut child = self.copy();
		let mut child2 = self.copy();
		for i in 0..partner.genes.len(){
			if i % 2 == 0{
				child.genes[i] = partner.genes[i];
			}else{
				child2.genes[i] = partner.genes[i];
			}
		}
		child.calculate_fitness();
		child2.calculate_fitness();
		if child.fitness > child2.fitness {
			child
		}else{
			child2
		}
	}

	fn mutate(&mut self, rate: f32) {
		let mut random = rand::thread_rng();
		for i in 0..self.genes.len() {
			let n = random.gen_range(0.0, 1.0);
			if n <= rate {
				self.genes[i] = random.gen_range(32, 126);
			}
		}
		self.calculate_fitness();
	}

}


struct Population {
	best: Vec<DNA>,
	person: i32,
	generation: i32,
	rate: f32
}

impl Population {

	fn new(person: i32, rate: f32) -> Population {
		Population{
			best: vec![],
			person: person,
			generation: 0,
			rate: rate
		}
	}

	fn populate(&mut self, target:String){
		for _ in 0..self.person {
			self.best.push(DNA::new(target.to_owned()));
		}

		let mut found = false;

		while !found {
			if let Some(dna) = self.best.first(){
				let mut c = 0;
				for (x,y) in dna.genes.iter().zip(dna.target.chars()){
					if *x == y as u8 {
						c+=1;
					}
				}

				if c == dna.target.len(){
					found = true
				}
			}

			for person in &mut self.best {
				person.calculate_fitness();
			}

			self.best.sort_by(|a,b| b.fitness.cmp(&a.fitness));

			for i in 0..self.best.len() - 1{
				let mut child = self.best[i].crossover(&self.best[i+1]);
				child.mutate(self.rate);
				self.best[i+1] = child;
				
			}
			self.best.sort_by(|a,b| b.fitness.cmp(&a.fitness));
			println!("Target : {}", self.best[0].target);
			println!("Matched : {}%", self.best[0].fitness * 100 / self.best[0].target.len() as i32);
			println!("Generation : {}", self.generation);
			println!("Population : {}", self.person);
			println!("{:?}", self.best.first().unwrap().genes.iter().map(|x| *x as char).collect::<String>());
			println!();
			self.generation+=1;
		}
	}
}

fn main() {
	Population::new(200, 0.01).populate("Genetic Algorithm.".to_owned());
}
