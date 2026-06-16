//! Vile Mutilator — `{5}{B}{B}` 6/5 Demon with Flying and Trample.
//!
//! Oracle:
//! * As an additional cost to cast this spell, sacrifice a creature or
//!   enchantment. (GAP: additional cast-time sacrifice costs are not
//!   expressible on a creature CardDefinition in this shape.)
//! * Flying, trample (keyword line).
//! * When this creature enters, each opponent sacrifices a nontoken
//!   enchantment of their choice, then sacrifices a nontoken creature of
//!   their choice.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vile Mutilator");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "As an additional cost to cast this spell, sacrifice a creature
    // or enchantment." — additional cast-time costs are not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_opponent_sacrifices,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Each opponent sacrifices a nontoken enchantment of their choice, then a
/// nontoken creature of their choice.
fn etb_each_opponent_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: ObjectFilter::permanent()
                .with_types(TypeLine::ENCHANTMENT.into())
                .nontoken(),
            count: 1,
        });
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: ObjectFilter::creature().nontoken(),
            count: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}
