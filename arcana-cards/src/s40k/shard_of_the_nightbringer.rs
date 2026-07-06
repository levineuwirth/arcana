//! Shard of the Nightbringer — `{5}{B}{B}{B}` 8/8 C'tan with Flying.
//!
//! Oracle:
//! * Flying.
//! * Drain Life — When this creature enters, if you cast it, target opponent
//!   loses half their life, rounded up. You gain life equal to the life lost
//!   this way.
//!
//! "if you cast it" (cast-vs-otherwise) is not a `conditions::` predicate —
//! GAP'd (fires unconditionally on ETB). The half-life drain is computed at
//! resolution from the target opponent's life total.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shard of the Nightbringer");
    let ctan = reg.interner_mut().intern("C'tan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ctan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![arcana_core::effects::KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if you cast it" not expressible; fires on every ETB.
            intervening_if: None,
            effect: drain_half_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_opponent()],
        }),
    )
}

fn drain_half_life(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let life = script::life(state, *p).max(0) as u32;
    let lost = (life + 1) / 2; // half, rounded up
    vec![
        Effect::LoseLife { player: *p, amount: lost },
        Effect::GainLife { player: trig.controller, amount: lost },
    ]
}
