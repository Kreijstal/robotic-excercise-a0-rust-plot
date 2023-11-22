#include "SpringMassWrapper.h"
#include "SpringMass.hpp" 
#include "SpringDamperMass.hpp" 

extern "C" {

SpringMassPtr SpringMass_create(double pos_init, double vel_init, double pos_eqm, double vel_eqm) {
    return new SpringMass(pos_init, vel_init, pos_eqm, vel_eqm);
}

void SpringMass_destroy(SpringMassPtr ptr) {
    delete static_cast<SpringMass*>(ptr);
}

int SpringMass_step(SpringMassPtr ptr) {
    return static_cast<SpringMass*>(ptr)->step();
}

int SpringMass_getConfiguration(SpringMassPtr ptr, int t, double* x, double* y) {
    Vec2d state;
    bool result = static_cast<SpringMass*>(ptr)->getConfiguration(t, state);
    if (result) {
        *x = state.x;
        *y = state.y;
    }
    return result ? 1 : 0;
}

int SpringMass_getCurrentSimulationTime(SpringMassPtr ptr) {
    return static_cast<SpringMass*>(ptr)->getCurrentSimulationTime();
}

SpringDamperMassPtr SpringDamperMass_create(double pos_init, double vel_init, double pos_eqm, double vel_eqm, double damping_coeff) {
    return new SpringDamperMass(pos_init, vel_init, pos_eqm, vel_eqm, damping_coeff);
}

void SpringDamperMass_destroy(SpringDamperMassPtr ptr) {
    delete static_cast<SpringDamperMass*>(ptr);
}

int SpringDamperMass_step(SpringDamperMassPtr ptr) {
    return static_cast<SpringDamperMass*>(ptr)->step();
}

}
