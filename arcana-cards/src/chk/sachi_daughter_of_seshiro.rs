//! Sachi, Daughter of Seshiro — `{2}{G}{G}` 1/3 Legendary Snake Shaman.
//!
//! * Other Snake creatures you control get +0/+1.
//!   Wired via a SelfEntersBattlefield trigger installing a
//!   `ContinuousEffect::filtered_pump` over Snake creatures you control,
//!   +0/+1, lasting while this creature is on the battlefield. (The "other"
//!   qualifier is a documented minor fidelity gap — Sachi is herself a Snake
//!   and matches the base-characteristics filter, so she self-includes.)
//! * Shamans you control have "{T}: Add {G}{G}."
//!   // GAP: granting a static ACTIVATED ability to a group of other
//!   permanents has no expressible form (only keyword grants exist as
//!   filtered continuous effects); unmodeled.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sachi, Daughter of Seshiro");
    let snake = reg.interner_mut().intern("Snake");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Shamans you control have '{T}: Add {G}{G}.'" — granting a static
    // activated mana ability to a group of permanents has no expressible form.

    reg.register(
        CardDefinition::new(name, chars)
            // "Other Snake creatures you control get +0/+1."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_snake_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn install_snake_anthem(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let snake = reg.interner().lookup("Snake").expect("Snake interned at register");
    let filter = ObjectFilter::creature()
        .with_subtype_sym(snake)
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            0,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
