//! Vodalian Mindsinger — `{1}{U}{U}` 2/2 Creature — Merfolk Wizard.
//!
//! * Kicker {1}{R} and/or {1}{G} — Kicker is not an expressible keyword
//!   (no `KeywordAbility::Kicker`), so it is GAP'd and `keywords` is empty.
//! * "This creature enters with two +1/+1 counters on it for each time it was
//!   kicked." — depends on the kick count, which is not tracked; GAP.
//! * "When this creature enters, gain control of target creature with power
//!   less than this creature's power for as long as you control this
//!   creature." — emitted as an ETB gain-control of a target creature
//!   (`ChangeControl`). The "power less than this creature's power" target
//!   restriction is a source-relative filter ObjectFilter can't express, and
//!   the "for as long as you control" duration linkage is not modeled; both
//!   are GAP'd, leaving a faithful (slightly over-broad) gain-control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vodalian Mindsinger");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    // GAP: Kicker {1}{R} and/or {1}{G}; enters-with-counters-per-kick scaling.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "power less than this creature's power" — no
                // source-relative power filter; targets any creature.
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_gain_control(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "for as long as you control this creature" duration linkage not
    // modeled — uses a permanent ChangeControl.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: trig.controller,
    }]
}
