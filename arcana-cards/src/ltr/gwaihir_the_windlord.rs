//! Gwaihir the Windlord — `{4}{W}{U}` Legendary 4/4 Bird Noble with Flying and
//! Vigilance.
//!
//! * "This spell costs {2} less to cast as long as you've drawn two or more
//!   cards this turn." — a conditional cost-reduction casting static; no
//!   expressible Effect form on a creature def (GAP).
//! * Flying, vigilance — base keywords.
//! * "Other Birds you control have vigilance." — a static keyword-granting
//!   anthem over Birds you control. Installed via a `SelfEntersBattlefield`
//!   trigger that adds a `filtered_keyword(Vigilance)` continuous effect
//!   anchored to Gwaihir with `Duration::WhileSourceOnBattlefield` (layer 6).
//!   (The filter matches base characteristics, so Gwaihir himself is included —
//!   a documented minor "OTHER" self-inclusion fidelity gap; he already has
//!   vigilance, so it is a no-op on him.)

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
    let name = reg.interner_mut().intern("Gwaihir the Windlord");
    let bird = reg.interner_mut().intern("Bird");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(noble);

    // GAP: "This spell costs {2} less to cast as long as you've drawn two or
    // more cards this turn" — conditional cost-reduction casting static, no
    // expressible Effect form on a creature def.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_bird_vigilance,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "Birds you control have vigilance", anchored to Gwaihir and
/// lasting until he leaves the battlefield.
fn install_bird_vigilance(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird = reg
        .interner()
        .lookup("Bird")
        .expect("Bird interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(bird);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            filter,
            KeywordAbility::Vigilance,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
