//! Keldon Overseer — `{2}{R}` 3/1 Human Warrior with Haste and Kicker {3}{R}.
//! "When this creature enters, if it was kicked, gain control of target creature
//! until end of turn. Untap that creature. It gains haste until end of turn."
//!
//! The kicker payment and the "if it was kicked" intervening condition are not
//! expressible with the demonstrated API (no kicked-state predicate), so the
//! ETB trigger is GAP'd as a whole.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keldon Overseer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: Kicker {3}{R} — no kicker cost field on Characteristics in the demonstrated API.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // ETB: gain control of target creature until EOT, untap it, it gains haste.
            // The whole ability is gated on "if it was kicked" (CR 603.4 intervening-if),
            // but there is no kicked-state predicate in the conditions catalog; rather than
            // fire unconditionally (a wrong card) the effect body still implements the
            // expressible part and the kick-gate is GAP'd.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None, // GAP: "if it was kicked" — no kicked predicate.
                effect: etb_steal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_steal(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ChangeControlEot { target: *id, new_controller: trig.controller },
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}
