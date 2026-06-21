//! Volcano Hellion — `{2}{R}{R}` 6/5 Hellion.
//!
//! * "Echo {X}, where X is your life total." — Echo is not a usable
//!   `KeywordAbility` variant; GAP'd.
//! * "When this creature enters, it deals an amount of damage of your choice
//!   to you and target creature. The damage can't be prevented." — the
//!   player-chosen variable damage amount has no expressible chooser primitive
//!   (damage amounts are fixed/scripted, not free-chosen), so this ETB is
//!   GAP'd rather than emitting a fixed-amount stand-in.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volcano Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);

    // GAP: "Echo {X}, where X is your life total" — Echo is not a usable keyword.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_choose_damage,
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

fn etb_choose_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "deals an amount of damage of your choice to you and target
    // creature, the damage can't be prevented" — a free player-chosen damage
    // amount has no chooser primitive, so the whole ETB damage is omitted.
    Vec::new()
}
