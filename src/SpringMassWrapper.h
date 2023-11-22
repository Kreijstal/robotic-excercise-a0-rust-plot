#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to represent SpringMass in C code
typedef void* SpringMassPtr;

// C-compatible wrappers
SpringMassPtr SpringMass_create(double pos_init, double vel_init, double pos_eqm, double vel_eqm);
void SpringMass_destroy(SpringMassPtr ptr);
int SpringMass_step(SpringMassPtr ptr);
int SpringMass_getConfiguration(SpringMassPtr ptr, int t, double* x, double* y);
int SpringMass_getCurrentSimulationTime(SpringMassPtr ptr);


typedef void* SpringDamperMassPtr;
SpringDamperMassPtr SpringDamperMass_create(double pos_init, double vel_init, double pos_eqm, double vel_eqm, double damping_coeff);
void SpringDamperMass_destroy(SpringDamperMassPtr ptr);
int SpringDamperMass_step(SpringDamperMassPtr ptr);


#ifdef __cplusplus
}
#endif
