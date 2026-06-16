//! Pattern of Rebirth — `{3}{G}` enchantment — Aura.
//! "Enchant creature.
//!  When enchanted creature dies, that creature's controller may search
//!  their library for a creature card, put that card onto the
//!  battlefield, then shuffle."
//!
//! Host-trigger Aura. An AttachedCreatureDoes { SelfDies } ability fires
//! when the enchanted creature dies; that creature's controller tutors a
//! creature card onto the battlefield. NOTE: TutorToBattlefield bundles
//! the shuffle; the "may" is a resolution-time choice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pattern of Rebirth");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: on_host_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // The dying creature's controller does the searching.
    let player = trig
        .dying_object()
        .and_then(|d| state.object_or_lki(d))
        .map(|o| o.controller)
        .unwrap_or(trig.controller);
    vec![Effect::TutorToBattlefield {
        player,
        filter: ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
        tapped: false,
    }]
}
