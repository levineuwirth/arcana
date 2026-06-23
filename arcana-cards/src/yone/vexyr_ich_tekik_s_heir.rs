//! Vexyr, Ich-Tekik's Heir — `{G}{W}{U}` 3/4 Legendary Creature —
//! Phyrexian Artificer.
//!
//! Oracle:
//! * Whenever you seek one or more cards, create a 3/3 colorless Phyrexian Golem
//!   artifact creature token. — There is no "you seek" `TriggerCondition`
//!   variant; the seek event is not in the demonstrated trigger catalog, so the
//!   trigger is GAP'd (no faithful condition to attach the token creation to).
//! * Golems you control have vigilance — STATIC continuous keyword anthem over
//!   your Golems. NOW WIRED via an ETB-installed
//!   `ContinuousEffect::filtered_keyword`.
//!
//! `Seek` is not a usable `KeywordAbility` variant, so no keyword is emitted.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Vexyr, Ich-Tekik's Heir");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let artificer = reg.interner_mut().intern("Artificer");
    // Intern "Golem" now so the effect fn's lookup is guaranteed to hit.
    let _golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: trigger — "Whenever you seek one or more cards, create a 3/3
    // colorless Phyrexian Golem artifact creature token" (no seek trigger
    // condition).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_golem_vigilance,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "Golems you control have vigilance", anchored to Vexyr.
fn etb_install_golem_vigilance(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg
        .interner()
        .lookup("Golem")
        .expect("Golem interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You)
                .with_subtype_sym(golem),
            KeywordAbility::Vigilance,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
