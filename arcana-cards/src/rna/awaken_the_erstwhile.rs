//! Awaken the Erstwhile — `{3}{B}{B}` sorcery. "Each player discards
//! all the cards in their hand, then creates that many 2/2 black
//! Zombie creature tokens."

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Awaken the Erstwhile");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards all the cards in their hand, then creates that many 2/2 black Zombie creature tokens.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        let n = script::hand_size(state, p);
        effects.push(Effect::Discard {
            player: p,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        });
        for _ in 0..n {
            effects.push(Effect::CreateToken { controller: p, token: token.clone() });
        }
    }
    effects
}
