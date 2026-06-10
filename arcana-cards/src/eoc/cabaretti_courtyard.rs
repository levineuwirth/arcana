//! Cabaretti Courtyard — nonbasic land (Streets of New Capenna, 2022).
//! "When this land enters, sacrifice it. When you do, search your
//! library for a basic Mountain, Forest, or Plains card, put it onto
//! the battlefield tapped, then shuffle and you gain 1 life."
//! The reflexive "When you do" trigger is flattened into a single ETB
//! effect sequence: sacrifice this land, fetch the basic, gain 1 life.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cabaretti Courtyard");
    // Pre-intern the fetchable basic-land subtypes for resolve-time lookup.
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sac_and_fetch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn etb_sac_and_fetch(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "When you do" trigger flattened into one effect
    // sequence; the self-sacrifice is modeled as a name-filtered
    // Effect::Sacrifice (no sacrifice-by-id effect exists).
    let basics: Vec<_> = ["Mountain", "Forest", "Plains"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: reg.interner().lookup("Cabaretti Courtyard"),
                types: Some(TypeLine::LAND.into()),
                ..ObjectFilter::default()
            },
            count: 1,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(
                    SupertypeSet::new().with(SupertypeSet::BASIC),
                )
                .with_subtypes_any(basics),
            tapped: true,
        },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ])]
}
