#include <mono-2.0/mono/jit/jit.h>
#include <mono-2.0/mono/metadata/mono-config.h>
#include <mono-2.0/mono/metadata/assembly.h>
MonoDomain *
mono_domain_create_appdomain_checked (char *friendly_name, char *configuration_file, MonoError *error);
void*(test)(int);
