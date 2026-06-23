//! Underworld Charger — `{2}{B}` 3/3 Nightmare Horse.
//! "This creature can't block."
//! "Escape—{4}{B}, Exile three other cards from your graveyard."
//! "This creature escapes with two +1/+1 counters on it." (GAP — Escape.)
//!
//! The "can't block" static is wired via a SelfEntersBattlefield trigger
//! installing a `ContinuousEffect::cant_block` self-restriction lasting
//! while this creature is on the battlefield. Escape is not a modeled
//! KeywordAbility variant and the escape-cast mechanic (cast from graveyard
//! for an exile-cards cost, entering with two +1/+1 counters) is not
//! expressible, so the keyword line is empty and the escape clauses are
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underworld Charger");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Escape — not a modeled KeywordAbility; the escape-cast
    // mechanic (cast from graveyard, exile three other cards, enter with
    // two +1/+1 counters) is not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't block." — self-restriction static.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cant_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_cant_block(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::cant_block(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
