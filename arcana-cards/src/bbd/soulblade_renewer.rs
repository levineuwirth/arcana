//! Soulblade Renewer — `{4}{G}` 2/2 Creature — Elf Warrior.
//!
//! * Keywords Support / Partner with / Partner are not expressible
//!   `KeywordAbility` variants — `keywords` is empty.
//! * "Partner with Soulblade Corrupter (When this creature enters, target
//!   player may put Soulblade Corrupter into their hand from their library,
//!   then shuffle.)" — a may-tutor-by-name into a TARGET player's hand;
//!   `TutorToHand` resolves only over the controller's own library (no target
//!   player), so this Partner-with ability is GAP'd.
//! * "When this creature enters, support 2. (Put a +1/+1 counter on each of up
//!   to two other target creatures.)" — emitted as an ETB targeting up to two
//!   creatures, each receiving a +1/+1 counter. ("Other" = excluding this
//!   creature; ObjectFilter can't exclude by source id, a minor fidelity gap.)

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soulblade Renewer");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Partner with Soulblade Corrupter" — may-tutor-by-name into a target
    // player's hand is not expressible (TutorToHand has no target player).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: support_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn support_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}
