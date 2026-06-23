//! Yuffie, Materia Hunter — `{2}{R}` 3/3 Legendary Human Ninja.
//! Ninjutsu {1}{R}.
//! "When Yuffie enters, gain control of target noncreature artifact for as
//! long as you control Yuffie. Then you may attach an Equipment you
//! control to Yuffie."
//!
//! Ninjutsu is not in the usable keyword surface (GAP'd). The ETB control
//! change targets a noncreature artifact; "for as long as you control
//! Yuffie" has no duration-linked control primitive, so it is modeled
//! with `Effect::ChangeControl` (permanent gain — a fidelity GAP on the
//! "for as long as" revert). The "Then you may attach an Equipment you
//! control to Yuffie" rider needs a player-chosen Equipment to attach
//! (no equip/attach pick over your own permanents) and is GAP'd.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yuffie, Materia Hunter");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Ninjutsu {1}{R} — Ninjutsu is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When Yuffie enters, gain control of target noncreature
            // artifact ... Then you may attach an Equipment ... (GAP)."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_take_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_take_artifact(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "gain control of target noncreature artifact for as long as you
    // control Yuffie" — modeled as a permanent control change (fidelity GAP
    // on the "for as long as" revert; no duration-linked control primitive).
    // GAP: "Then you may attach an Equipment you control to Yuffie." — needs
    // a chosen Equipment from your own permanents; no attach pick available.
    vec![Effect::ChangeControl { target: *id, new_controller: trig.controller }]
}
