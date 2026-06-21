//! Artisan of Kozilek — `{9}` 10/9 Eldrazi.
//! "When you cast this spell, you may return target creature card from your
//!  graveyard to the battlefield."
//! "Annihilator 2 (Whenever this creature attacks, defending player sacrifices two
//!  permanents of their choice.)"
//!
//! The cast trigger is a GAP — there is no "when you cast this spell" trigger
//! condition. Annihilator 2 is not in the usable KeywordAbility surface but is
//! modeled as a SelfAttacks trigger making the defending player sacrifice two
//! permanents (fully wired via Effect::Sacrifice).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Artisan of Kozilek");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(9)),
        // GAP: keyword — Annihilator 2 not in the usable KeywordAbility surface;
        // modeled as a SelfAttacks trigger below.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: triggered ability — "When you cast this spell, you may return target
    // creature card from your graveyard to the battlefield." There is no cast
    // trigger condition (no SelfCast / "when you cast this spell" variant), so it
    // is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: annihilator_2,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn annihilator_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defender) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Sacrifice {
        player: defender,
        filter: ObjectFilter::permanent(),
        count: 2,
    }]
}
