#ifndef ENGINE_HPP_
#define ENGINE_HPP_

#include <functional>
#include <vector>

#include "algorithm.hpp"

class Engine {
	int hold;
	double cash;
	std::vector<double>& data;
	Algorithm algorithm;

	public:
		Engine(std::vector<double>& data, Algorithm algorithm);
		void run();
	
	private:
};

#endif