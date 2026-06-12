//! Brutal Cathar // Moonrage Brute — `{2}{W}` Creature — Human Soldier Werewolf 2/2.
//!
//! Front face (Brutal Cathar):
//! "Whenever this creature enters or transforms into Brutal Cathar, exile target
//! creature an opponent controls until this creature leaves the battlefield."
//! Daybound.
//!
//! Back face (Moonrage Brute — Creature — Werewolf 3/3):
//! First strike. Ward—Pay 3 life. Nightbound.
//!
//! GAPs:
//! - "Daybound" / "Nightbound" are layout/day-night keywords not in the
//!   implemented KeywordAbility list — omitted; the day/night transform
//!   automation is engine debt.
//! - "Transform" is a layout keyword, not a KeywordAbility.
//! - The exile is wired via `Effect::ExileUntilSourceLeaves` — the engine
//!   returns the card when this creature leaves the battlefield.
//! - "Ward—Pay 3 life" is a non-mana Ward cost, which is not expressible as
//!   `KeywordAbility::Ward(ManaCost)`; omitted from the back face keywords.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brutal Cathar");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Daybound not in keyword list
        keywords: vec![],
        ..Default::default()
    };

    // Back face — Moonrage Brute (Creature — Werewolf 3/3)
    let back_name = reg.interner_mut().intern("Moonrage Brute");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // First strike modeled; GAP: Ward—Pay 3 life (non-mana) and Nightbound omitted
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "Whenever this creature enters or transforms into Brutal Cathar,
            // exile target creature an opponent controls until this creature
            // leaves the battlefield." Enters half:
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            // Transforms-into-front half:
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: etb_exile_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_exile_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // O-Ring linkage: the engine returns the exiled creature when this
    // creature leaves the battlefield.
    vec![Effect::ExileUntilSourceLeaves {
        source: trig.source,
        target: *id,
    }]
}
