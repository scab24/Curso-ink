#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

#[ink::contract]
mod organization {

    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};

    // Objetivo de este contrato:
    // Almacenamiento:
    // - Mantener una lista de miembros con su reputación.
    // - Incluir una cuenta administradora con permisos para gestionar miembros.

    // Mensajes:
    // - Agregar o eliminar un miembro.
    // - Votar (solo un miembro puede votar a otro).
    // - Consultar la reputación de un miembro.

    #[ink(event)]
    pub struct NewMember {
        #[ink(topic)]
        member: MemberInfo
    }

    #[ink(event)]
    pub struct Vote {
        #[ink(topic)]
        member: MemberInfo
    }

    #[ink(event)]
    pub struct MemberReputation {
        #[ink(topic)]
        reputation: u32
    }

    #[ink(storage)]
    pub struct Organization {
        admin: AccountId,
        members: Vec<MemberInfo>,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct MemberInfo {
        member_id: AccountId,
        reputation: u32
    }

    impl Organization {
        // Constructor para crear una nueva instancia del contrato
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let members_vector: Vec<MemberInfo> = Vec::new();
            Self { 
                admin,
                members: members_vector
            }
        }

        // Función para agregar un miembro a la organización
        #[ink(message)]
        pub fn add_member(&mut self, member: MemberInfo) {
            assert!(self.env().caller() == self.admin);
            self.members.push(member.clone());
            self.env().emit_event(NewMember { member });
        }

        // Función para agregar varios miembros a la organización
        #[ink(message)]
        pub fn add_members(&mut self, members: Vec<MemberInfo>) {
            assert!(self.env().caller() == self.admin);
            for item in members {
                let member = MemberInfo {member_id: item.member_id, reputation: item.reputation };
                self.members.push(member.clone());
                self.env().emit_event(NewMember { member });
            }
        }

        // Función para que un miembro vote por otro
        #[ink(message)]
        pub fn vote(&mut self, id: AccountId) {
           assert!(self.is_member(id));
           let member = self.vote_member(id);
           self.env().emit_event(Vote { member });
        }

        // Función para consultar la reputación de un miembro
        #[ink(message)]
        pub fn reputation(&mut self, id: AccountId) {
            assert!(self.is_member(id));
            let member: MemberInfo = self.get_member(id);
            let reputation = member.reputation;
            self.env().emit_event(MemberReputation { reputation });
        }

        // Función para obtener la dirección de la cuenta del contrato
        #[ink(message)]
        pub fn get_contract_address(&self) -> AccountId {
            self.env().account_id()
        }

        // Función interna para verificar si una cuenta es un miembro
        fn is_member(&mut self, id: AccountId)-> bool {
            let mut result:bool = false;
            for item in &self.members {
                if item.member_id == id {
                    result = true;
                    break;
                }
            }
            result
        } 
       
        // Función interna para que un miembro vote por otro
        fn vote_member (&mut self, id: AccountId) -> MemberInfo {
            let mut member: MemberInfo = MemberInfo {member_id: id, reputation: 0};
            for item in &mut self.members {
                if item.member_id == id {
                    item.reputation += 1;
                    member = item.clone();
                    break
                }
            }
            assert!(id == member.member_id);
            assert!(member.reputation > 0);
            member
        }

        // Función interna para obtener información de un miembro
        fn get_member (&mut self, id: AccountId) -> MemberInfo {
            let mut member: MemberInfo = MemberInfo {member_id: id, reputation: 0};
            for item in &self.members {
                if item.member_id == id {
                    member = item.clone();
                    break
                }
            }
            assert!(id == member.member_id);
            assert!(member.reputation > 0);
            member
        }
    }
}
