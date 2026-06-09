//! Fungal Rebirth — `{2}{G}` instant. "Return target permanent card from
//! your graveyard to your hand. If a creature died this turn, create two
//! 1/1 green Saproling creature tokens." The died-this-turn conditional is
//! checked via `conditions::a_creature_died_this_turn`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fungal Rebirth");
    let _saproling = reg.interner_mut().intern("Saproling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target permanent card from your graveyard to your hand. If a creature died this turn, create two 1/1 green Saproling creature tokens.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::permanent(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::ReturnFromGraveyardToHand { target: *id }];
    // "If a creature died this turn, create two 1/1 green Saproling
    // creature tokens."
    if arcana_core::conditions::a_creature_died_this_turn(state) {
        let saproling = reg
            .interner()
            .lookup("Saproling")
            .expect("Saproling interned during register()");
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(saproling);
        let token = TokenDefinition {
            name: saproling,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        };
        for _ in 0..2 {
            effects.push(Effect::CreateToken {
                controller: entry.controller,
                token: token.clone(),
            });
        }
    }
    effects
}
