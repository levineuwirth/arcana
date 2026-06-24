//! Roiling Horror — `{3}{B}{B}` */* Horror.
//!
//! * "Roiling Horror's power and toughness are each equal to your life
//!   total minus the life total of an opponent with the most life." —
//!   wired at Layer 7a via a SelfEntersBattlefield self_pt_cda whose compute
//!   returns `(diff, diff)` where diff = your life minus the max opponent
//!   life. Bones `*/*` (PtValue::Star).
//! * Suspend X—{X}{B}{B}{B}. — GAP: the `Suspend` keyword is not in the
//!   modeled KeywordAbility surface (`keywords` stays empty).
//! * "Whenever a time counter is removed from this card while it's
//!   exiled, target player loses 1 life and you gain 1 life." — GAP: there
//!   is no counter-removal / time-counter-while-exiled trigger condition.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roiling Horror");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/*` — value defined by the CDA installed below.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: P/T each equal to your life total minus the life total
/// of an opponent with the most life.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            life_differential_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = your life total minus the life total of an opponent with the most
/// life.
fn life_differential_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let my_life = script::life(s, who);
    let max_opp_life = script::opponents(s, who)
        .into_iter()
        .map(|p| script::life(s, p))
        .max()
        .unwrap_or(0);
    let diff = my_life - max_opp_life;
    (diff, diff)
}
