use bitflags::bitflags;

use crate::extractors::perms::{RequireBrandPerm, RequireItemPerm, RequireModelPerm, RequireProjectPerm, RequireUserPerm};

macro_rules! define_perms {
    ($prefix:ident, $struct_name:ident, [$(($flag:ident, $suffix:ident, $shift:expr)),+ $(,)?]) => {
        paste::paste! {
            $(
                pub const [<$prefix _PERM_ $suffix>]: i32 = 1 << $shift;
            )+

            bitflags! {
                #[derive(Clone, Copy)]
                pub struct $struct_name: i32 {
                    $(const $flag = [<$prefix _PERM_ $suffix>];)+
                }
            }

            pub const [<$prefix _PERM_ALL>]: i32 = $([<$prefix _PERM_ $suffix>])|+;
        }
    };
}

define_perms!(I, ItemPermissions, [
    (WRITE,   WRITE,   0),
    (READ,    READ,    1),
    (MOD,     MOD,     2),
    (DEL,     DELETE,  3),
    (WEBHOOK, WEBHOOK, 4),
    (ASIGN,   ASSIGN,  5),
]);

define_perms!(U, UserPermissions, [
    (WRITE, WRITE,  0),
    (READ,  READ,   1),
    (MOD,   MOD,    2),
    (DEL,   DELETE, 3),
]);

define_perms!(P, ProjectPermisions, [
    (WRITE, WRITE, 0),
    (READ,  READ,  1),
    (MOD,   MOD,   2),
]);

define_perms!(B, BrandPermissions, [
    (WRITE, WRITE, 0),
    (READ,  READ,  1),
    (MOD,   MOD,   2),
]);

define_perms!(M, ModelPermissions, [
    (WRITE, WRITE, 0),
    (READ,  READ,  1),
    (MOD,   MOD,   2),
]);

macro_rules! perm_alias {
    ($extractor:ident => $($alias:ident = $const_name:ident : $bits:expr;)+) => {
        $(
            pub const $const_name: i32 = $bits;
            pub type $alias = $extractor<$const_name>;
        )+
    };
}

perm_alias!(RequireUserPerm =>
    CanManageUsers = CAN_MANAGE_USERS_B: U_PERM_READ | U_PERM_WRITE | U_PERM_MOD;
    CanReadUsers    = CAN_READ_USERS_B: U_PERM_READ;
);

perm_alias!(RequireProjectPerm =>
    CanManageProjects = CAN_MANAGE_PROJECTS_B: P_PERM_MOD | P_PERM_READ | P_PERM_WRITE;
    CanReadProjects    = CAN_READ_PROJECTS_B: P_PERM_READ;
);

perm_alias!(RequireBrandPerm =>
    CanManageBrands = CAN_MANAGE_BRANDS_B: B_PERM_MOD | B_PERM_READ | B_PERM_WRITE;
    CanReadBrands    = CAN_READ_BRANDS_B: B_PERM_READ;
);

perm_alias!(RequireModelPerm =>
    CanManageModels = CAN_MANAGE_MODELS_B: M_PERM_MOD | M_PERM_READ | M_PERM_WRITE;
    CanReadModels    = CAN_READ_MODELS_B: M_PERM_READ;
);

perm_alias!(RequireItemPerm =>
    CanManageItems = CAN_MANAGE_ITEMS_B: I_PERM_MOD | I_PERM_READ | I_PERM_WRITE;
    CanReadItems   = CAN_READ_ITEMS_B: I_PERM_READ;
);
