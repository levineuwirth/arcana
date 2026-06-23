//! Scrapshooter — `{1}{G}{G}` 4/4 Raccoon Archer with Reach.
//!
//! Oracle:
//! * Gift a card (promise an opponent a gift on cast; if you do, when it
//!   enters, they draw a card).
//! * Reach
//! * When this creature enters, if the gift was promised, destroy target
//!   artifact or enchantment an opponent controls.
//!
//! Reach is a base keyword. The Gift mechanic (promise-a-gift cast rider +
//! the opponent-draws-on-ETB clause) has no engine primitive — GAP. The
//! ETB destroy ability IS wired (target artifact or enchantment an opponent
//! controls), but its "if the gift was promised" intervening-if gate is
//! unmodeled (Gift is unrepresented), so it fires unconditionally — a
//! documented fidelity GAP. (No conditions:: helper can read "gift promised".)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scrapshooter");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        // GAP: Gift a card — no engine primitive for the promise-a-gift
        // cast rider or the opponent-draws-on-ETB clause.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if the gift was promised" — Gift is unmodeled, no
                // condition can read it; fires unconditionally.
                intervening_if: None,
                effect: etb_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(
                                TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                            ))
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_destroy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
