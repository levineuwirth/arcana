//! Mendicant Core, Guidelight — `{W}{U}` */3 Legendary Artifact Creature — Robot.
//!
//! * Mendicant Core's power is equal to the number of artifacts you control.
//!   (wired via a self-CDA installed on ETB, ContinuousEffect::self_pt_cda at
//!   Layer 7a; only POWER is `*`, so the scalar compute returns the artifact
//!   count for power and the printed fixed `3` toughness.)
//! * Start your engines! / speed mechanic. (GAP — not modeled)
//! * Max speed — Whenever you cast an artifact spell, you may pay {1}. If you
//!   do, copy it. (GAP)
//!
//! The speed mechanic ("Start your engines!", Max speed) is unmodeled. The
//! Max-speed copy trigger is GAP'd: it is gated on having max speed
//! (unexpressible) and `CopySpell` needs the just-cast spell's id, which a
//! `SpellCast` trigger exposes no accessor for.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mendicant Core, Guidelight");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Start your engines!" and the speed mechanic are not modeled.
    // GAP: "Max speed — Whenever you cast an artifact spell, you may pay {1}.
    //       If you do, copy it." — gated on max speed (unexpressible) and
    //       CopySpell has no accessor for the just-cast spell's id from a
    //       SpellCast trigger.

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

/// Layer 7a self-CDA. Only POWER is `*` (= the number of artifacts you
/// control); toughness is the printed fixed `3`.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = number of artifacts you control; toughness = printed 3.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(s, &filter, who) as i32;
    (n, 3)
}
