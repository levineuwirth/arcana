//! Archfiend of Spite — `{5}{B}{B}` 6/6 black Demon with Flying.
//! "Whenever a source an opponent controls deals damage to this creature,
//! that source's controller loses that much life unless they sacrifice that
//! many permanents." — GAP'd: can't recover the damaging source's controller,
//! and "sacrifice N or lose N life" isn't an expressible payment gate.
//! Madness {3}{B}{B} is GAP'd (not in the usable keyword surface).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Archfiend of Spite");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    // GAP: keyword Madness {3}{B}{B} — not in the usable keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
            intervening_if: None,
            effect: dealt_damage_punish,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dealt_damage_punish(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that source's controller loses that much life unless they
    // sacrifice that many permanents of their choice" — no accessor exposes
    // the damaging source's controller, and the sacrifice-or-lose-life choice
    // gate (variable-count sacrifice as an alternative to life loss) isn't
    // expressible with the demonstrated payment/effect surface.
    Vec::new()
}
