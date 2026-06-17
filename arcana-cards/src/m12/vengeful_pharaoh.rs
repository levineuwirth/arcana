//! Vengeful Pharaoh — `{2}{B}{B}{B}` 5/4 black Zombie with Deathtouch.
//!
//! Oracle:
//! * Deathtouch (keyword).
//! * Whenever combat damage is dealt to you or a planeswalker you control, if
//!   this card is in your graveyard, destroy target attacking creature, then
//!   put this card on top of your library.
//!
//! The trigger is graveyard-zoned (`trigger_zones: [Graveyard]`), so the "if
//! this card is in your graveyard" intervening-if is structurally satisfied by
//! the firing zone — `intervening_if: None`. "to you or a planeswalker you
//! control" is approximated as combat damage to the player (planeswalker side
//! is a fidelity gap). The two resolution effects (destroy target attacking
//! creature; put this on top of library) post no player choices, so they ride
//! as a bare two-element vec.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Pharaoh");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: destroy_attacker_recur,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().attacking_only(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn destroy_attacker_recur(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return vec![Effect::PutOnTopOfLibrary { target: trig.source }];
    };
    // GAP (fidelity): trigger should also fire when a planeswalker you control
    // takes combat damage; only the player side is modeled via TargetFilter::Player.
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::PutOnTopOfLibrary { target: trig.source },
    ]
}
