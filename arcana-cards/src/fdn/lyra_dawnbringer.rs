//! Lyra Dawnbringer — `{3}{W}{W}` 5/5 Legendary Angel with Flying, First strike,
//! Lifelink.
//! "Other Angels you control get +1/+1 and have lifelink."
//!
//! The keyword line is base characteristics. The static anthem ("Other Angels
//! you control get +1/+1 and have lifelink") is installed via a
//! `SelfEntersBattlefield` trigger that adds two continuous effects anchored to
//! this creature with `Duration::WhileSourceOnBattlefield`:
//! a `filtered_pump` (+1/+1, layer 7c) and a `filtered_keyword(Lifelink)`
//! (layer 6), both over Angels you control. (The filter matches base
//! characteristics, so Lyra herself is included — a documented minor "OTHER"
//! self-inclusion fidelity gap.)

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
    let name = reg.interner_mut().intern("Lyra Dawnbringer");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_angel_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "Angels you control get +1/+1 and have lifelink", anchored to
/// Lyra and lasting until she leaves the battlefield.
fn install_angel_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let angel = reg
        .interner()
        .lookup("Angel")
        .expect("Angel interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(angel);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Lifelink,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
