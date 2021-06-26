/* engine.cpp */

#include <functional>
#include <iostream>
#include <vector>

#include "engine.hpp"
#include "algorithm.hpp"


Engine::Engine(std::vector<double>& data, Algorithm algorithm) 
: data(data), algorithm(algorithm) {
	this->hold = 0;
	this->cash = 0.0;
}

void
Engine::run() {
	size_t idx = 0;
	auto start = data.begin();
	for(size_t idx = 0; idx < data.size(); ++idx) {
		auto target = algorithm.run(
			start, start + idx
		);
	}
}
