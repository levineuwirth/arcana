//! Ghastly Death Tyrant — `{4}{B}{B}` 6/5 black Beholder Skeleton.
//! "When this creature enters, choose one —
//!  • Disintegration Ray — Destroy target enchantment an opponent controls.
//!    You lose life equal to its mana value.
//!  • Death Ray — Creatures you control gain deathtouch until end of turn."
//!
//! Triggered abilities have no modal dispatch in this engine surface, so we
//! implement the non-targeted Death Ray mode (grant deathtouch to your
//! creatures) and GAP the modal choice + the Disintegration Ray mode (which
//! also needs "lose life equal to mana value", not derivable post-destroy).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghastly Death Tyrant");
    let beholder = reg.interner_mut().intern("Beholder");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beholder);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: modal "choose one" ETB has no triggered-ability modal dispatch;
            // implementing only the Death Ray mode. The Disintegration Ray mode
            // (destroy target enchantment + lose life equal to its mana value) is omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: death_ray,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn death_ray(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: arcana_core::objects::NULL_OBJECT_ID,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        }),
    }]
}
