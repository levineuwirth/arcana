//! Pia Nalaar, Consul of Revival — `{R}{W}` 2/3 Legendary Human Artificer.
//!
//! "Thopters you control have haste." — WIRED as an ETB-installed
//!  `ContinuousEffect::filtered_keyword(Haste)` (filter: Thopters you control),
//!  lasting while Pia Nalaar is on the battlefield (glorious_anthem precedent).
//! "Whenever you play a land from exile or cast a spell from exile, create a 1/1
//!  colorless Thopter artifact creature token with flying." — there is no
//!  TriggerCondition variant for play/cast FROM EXILE (GAP).

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
    let name = reg.interner_mut().intern("Pia Nalaar, Consul of Revival");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    // Intern the anthem-filter subtype.
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: trigger "Whenever you play a land from exile or cast a spell from
    // exile, create a 1/1 Thopter" — no matching TriggerCondition variant for a
    // play/cast-from-exile event.
    reg.register(
        CardDefinition::new(name, chars)
            // "Thopters you control have haste." installed on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_thopter_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Thopters you control have haste" anchored to Pia Nalaar, lasting
/// while it remains on the battlefield.
fn install_thopter_haste(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let thopter = reg.interner().lookup("Thopter").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(thopter);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            filter,
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
