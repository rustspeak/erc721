#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc721 {
    use ink_prelude::collections::HashMap;
    use  ink::Language;
    use ink::prelude::*;
    use ink::core::env::caller;


    #[ink(storage)]
    pub struct Erc721 {
        operator_approvals : Lazy<HashMap<AccountId, HashMAp<AccountId, bool>>>,
        token_owners:  HashMap<u32, AccountId>,
        token_uris: HashMap<u32, String>,
        token_names: HashMap<u32, String>,
        total_supply : u32,
        
    }
    pub  struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        token_id: u32,
    }

    impl Erc721 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                token_owners: HashMap::new(),
                token_uris: HashMap::new(),
                token_names: HashMap::new(),
                total_supply : 0,
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()

        } 
        /*  permet  a un proprietaire  de donner 
          de donner  une  autorisation  a un autre  compte  
          pour gerer  un  jeton  specifique */
        #[ink(message)]
        pub fn approve( &mut self,
             #[unk(param)] to: AccountId, 
             #[ink(param)] tokenId : u64 ) {
            //  verifier  si l'expediteur   est  le  proprietaire du jeton
                let owner =  self.owmers.get(&tokenId);
                    match owner {
                        Some(owner) => {
                            assert!(owner == &caller(), "only the  owner can approve");
                            // Stoker  l'approbation 
                            let mut approvals = self.approvals.get_mut(&tokenId);
                            match approvals {
                                Some(approvals) => {
                                    approvals.insert(to, caller());
                                }
                                None => {
                                    let mut new_approvals = HashMap::new();
                                    new_approvals.insert(to, caller());
                                    self.approvals.insert(tokenId, new_approvals);
                                }
                            }

                        }
                        None => Err("Token ID does not exist")
                    }                    
                }
            
    //  pour  compter  les  jeton  du  proprietaire 
        #[ink(message)]
        pub fn balance_of(&self,  #[ink(param)] owner : AccountId ) -> balance {

           // iterez sur  tout les jetons  du contract 
            let mut balance = 0;
            for (token_id, owner) in self.token_owners.iter() {
                if owner == &owner {
                    balance += 1;
                }
            }
            balance                 
        }

        /* fonction pour  permettrent  au proprietaire d'un jeton  de 
        detruire definitivement la  blockchain  */

        #[ink(message)] 
        pub fn burn (&mut  self, #[ink(param)]  tokenId: u64 ){
            // verifier si l'expediteur est le proprietaire du jeton
            let owner =  self.owners.get(&tokenId);
            match owner {
                Some(owner) => {
                    assert!(owner == &caller(), "only the owner can burn");
                    // message d'avertissement  pour  dir  que  le  jeton est  bruler  definitivement 
                    log!("Attention ! cette  action  detruira  definitivement   le  jeton {}",  tokenId);
                    // supprimer  le jeton  du mapping  proprietaires
                    self.owners.remove(&tokenId);
                    //  emettre un evenement de burn
                    emit!(Burn{
                        owner: *owner,
                        token_id,
                    });
                }
                None => Err("Token ID does not exist"),
            }
        }

        #[ink(message)]
        pub fn  get_approved(&self, #[ink(param)] tokenId : u64) -> Option<AccountId> {
            // recuperer  le  Hashap d'approbation pour le  jeton donne 
            let approvals = self.approvals.get(&tokenId);
            // si le  hashamp existe rencoyer l'addresse  de l'approbation
            approvals.map(|approvals|  approvals.get(&caller()),cloned());
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, 
                                    #[ink(param)] owner : AccounId,
                                    #[ink(param)]  operator : AccounId) -> bool {
                                        // recuperez la carte  d;approbation de l'operateur pour le  proprietaire donne
                                        let operator_approvals =  self.operator_approvals.get(&owner);
                                        // si la  carte d'approtion  existe , verifier  si l'operateur  est  aprouve
                                        operator_approvals.map_or(false | operator_approvals| {
                                            operator_approvals.get(&operator).cloned().unwrap_or(false)
                                        })
                                    }
        #[ink(message)]
        pub fn  event_is_approved_fro_all(&self, #[ink(param)] owner : AccountId, #[ink(param)]  operator : AccountId )  -> bool {

            //  evenement  pour indiquer  le   resultat de la verification
            emit!(Approval_for_all(owner, operator, approved));

            approved
        }
        
        // perment   a un  createur   de  jeton  de  cree un nouvveau jeton et de l'attribuer a un compte  specifique
        #[ink(message)]
        pub fn mint(&mut  self , #[ink(param)] to: AccountId, #[ink(param)] tokenId  :  u64) {
            // verifier  si l'expediteur  est  un  createur   de jetons
            assert!(self.is_creator(&caller()),  "Only  the  creator  can mint");
            // verifier  si le jeton  existe  deja
            aasert!(!self.owers.contains_key(&tokenId), "token Id already exists");
            // incrementerl e  nombre  total  de jeton 
            let  total_supply = self.total_supply.get_mut();
            *total_supply += 1;

            // attribuer  les  jeton  au destinataire 
            self.owner.insert(tokenId, to);

            // evenement  pour le  minage 
            emit!(Mint{
                to,
                token_id,
            });
        }

        // fonction   pour   verifier  si le  creatuer   du jetond  a  implementer 
        fn is_creator(&self, account : &Account) -> bool {
            account == &self.creator_adress 
        }

        fn is_creator(&self, account: &AccountId) -> bool {
            self.role_manager.get(account).map_or(false, |roles| roles.contains(&Role::Creator))
        }
        //  fonction   pour renvoyer  l'adresse  du rpoprietaire actuel  d'un jeton specifique 

        #[ink(message)]
        pub fn  owner_of(&self, #[ink(param)] tokenId : u64, ) -> Result<AccountId> {
            // recuperer  le proprietaire  du jeton  donner 
            self.owners.get(&tokenId)
            // sil le  proprietaire  exite revoyer-le
            .map(|owner| owner.clone())
            // si il n'existe  pas  renvoie une  erreur 
            .ok_or(Error::TokenNotFound)
        }
        #[ink(message)]
        pub fn  mint_true(&self){
            // verifier  si l'expediteur  est  un  createur   de jetons
            assert!(self.is_creator(&caller()),  "Only  the  creator  can mint");
            // verifier  si le jeton  existe  deja
            aasert!(!self.owers.contains_key(&tokenId), "token Id already exists");
            // incrementerl e  nombre  total  de jeton 
            let  total_supply = self.total_supply.get_mut();
            *total_supply += 1;

            // attribuer  les  jeton  au destinataire 
            self.owner.insert(tokenId, to);

            // evenement  pour le  minage 
            emit!(Mint{
                to,
                token_id,
            });

        }




    }
    pub enum Event {
        Mint {
            owner : AccountId,
            token_id : u64,
        },
    }

fn is_creator(&self, account: &AccountId) -> bool {
    self.role_manager.get(account).map_or(false, |roles| roles.contains(&Role::Creator))
}




    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let erc721 = Erc721::default();
            assert_eq!(erc721.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc721 = Erc721::new();
            assert_eq!(erc721.get(), false);
            erc721.flip();
            assert_eq!(erc721.get(), true);
        }
        #[ink::test]
        fn works_burn(){
            let mut erc721 = Erc721::new();
            assert_eq!(erc721.get(), false);
            erc721.flip();
            assert_eq!(erc721.get(), true);
            erc721.burn();
            assert_eq!(erc721.get(), false);
        }
        #[ink::test]
        fn works_mint(){
            // il est  question  de realiser  un vrai  test  rude  pour  cette  focntion
            // car  il  n'existe  pas  de  fonction  pour  verifier  si  le  jeton  a  ete  cree
            let mut erc721 = Erc721::new();
            assert_eq!(erc721.get(), false);
            erc721.flip();
            assert_eq!(erc721.get(), true);
            erc721.burn();
            assert_eq!(erc721.get(), false);
            erc721.mint();
            assert_eq!(erc721.get(), true);
        }
    }
}
