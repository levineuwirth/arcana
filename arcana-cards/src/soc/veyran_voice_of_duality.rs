//! Veyran, Voice of Duality — `{1}{U}{R}` 2/2 Legendary Efreet Wizard.
//! "Magecraft — Whenever you cast or copy an instant or sorcery spell,
//! Veyran gets +1/+1 until end of turn."
//! "If you casting or copying an instant or sorcery spell causes a
//! triggered ability of a permanent you control to trigger, that ability
//! triggers an additional time." (static — GAP'd)
//!
//! The Magecraft trigger is wired as a SpellCast trigger filtered to
//! instant-or-sorcery spells you cast; the "or copy" half is not separately
//! observable, so only the cast leg fires.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Veyran, Voice of Duality");
    let efreet = reg.interner_mut().intern("Efreet");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "...causes a triggered ability of a permanent you control to
    // trigger, that ability triggers an additional time" — a static
    // ability-doubling rule with no expressible primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                    TypeLine::INSTANT | TypeLine::SORCERY,
                ))),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: magecraft_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn magecraft_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
