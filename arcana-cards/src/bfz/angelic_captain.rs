//! Angelic Captain — `{3}{R}{W}` 4/3 Creature — Angel Ally. R/W.
//! Flying.
//! "Whenever this creature attacks, it gets +1/+1 until end of turn for each
//! other attacking Ally." — SelfAttacks → count attacking Allies (minus
//! itself, since it's an attacking Ally) and Pump self by that much.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angelic Captain");
    let angel = reg.interner_mut().intern("Angel");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_per_other_ally,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_per_other_ally(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Count attacking Allies (this creature is one), then subtract itself for
    // "each OTHER attacking Ally".
    let total = script::count_matching(
        state,
        &script::subtype_filter(reg, "Ally").attacking_only(),
        trig.controller,
    );
    let others = total.saturating_sub(1) as i32;
    if others == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: others,
        toughness: others,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
