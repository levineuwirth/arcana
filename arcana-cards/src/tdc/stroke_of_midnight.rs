//! Stroke of Midnight — `{2}{W}` instant. "Destroy target nonland permanent.
//! Its controller creates a 1/1 white Human creature token." We can target a
//! nonland permanent; "its controller creates a token" requires reading the
//! target's controller at resolve time, but `script::*` exposes no
//! `controller_of(state, id)`. GAP the token rider's controller binding
//! (token is created under the spell's controller as the closest stand-in).

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stroke of Midnight");
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target nonland permanent. Its controller creates a 1/1 white Human creature token.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    let human = reg.interner().lookup("Human").expect("Human interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    // GAP: no script::controller_of(state, id); token's "its controller" is the spell's controller as a stand-in.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: human,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
