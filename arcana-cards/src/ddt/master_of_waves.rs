//! Master of Waves — `{3}{U}` 2/1 Merfolk Wizard.
//!
//! Oracle:
//! * Protection from red (keyword not expressible — GAP)
//! * Elemental creatures you control get +1/+1. (static anthem — GAP)
//! * When this creature enters, create a number of 1/0 blue Elemental creature
//!   tokens equal to your devotion to blue.
//!
//! Protection is not in the usable keyword surface, so `keywords` is empty and
//! the keyword is GAP'd. The Elemental anthem static is GAP'd. The ETB token
//! creation is faithful: devotion to blue (script::devotion) drives how many
//! 1/0 blue Elemental tokens are minted.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Master of Waves");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the token's subtype so the resolver can rebuild it.
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword — Protection from red is not expressible.
        ..Default::default()
    };

    // GAP: static — "Elemental creatures you control get +1/+1."
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_elementals,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_elementals(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::devotion(state, trig.controller, ColorSet::blue());
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: elemental,
                colors: ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(0)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
