//! Kastral, the Windcrested — `{3}{W}{U}` 4/5 Legendary Bird Scout.
//!
//! Flying.
//! Whenever one or more Birds you control deal combat damage to a
//! player, choose one —
//! • You may put a Bird creature card from your hand or graveyard onto
//!   the battlefield with a finality counter on it.
//! • Put a +1/+1 counter on each Bird you control.
//! • Draw a card.
//!
//! The trigger is modelled as a `DamageDealt` from a Bird source you
//! control to a player (combat only). Triggered abilities have no modal
//! field, so the "choose one" selection is GAP'd; the resolver emits the
//! third, fully-faithful mode (draw a card). Modes 1 and 2 are noted as
//! the GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kastral, the Windcrested");
    let bird = reg.interner_mut().intern("Bird");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(scout);

    let bird_src = script::subtype_filter(reg, "Bird").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: bird_src,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: kastral_choose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn kastral_choose(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose one —" modal on a triggered ability is not expressible
    // (no modal field on TriggeredAbilityDef). Mode 1 (put a Bird from
    // hand/graveyard with a finality counter) and mode 2 (+1/+1 counter on
    // each Bird you control) are dropped; the resolver emits mode 3.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
