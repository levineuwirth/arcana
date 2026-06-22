//! Inquisitor Greyfax — `{1}{W}{U}{B}` 3/3 Legendary Creature — Human Inquisitor.
//!
//! Vigilance.
//! Static "Unquestionable Wisdom — Other creatures you control get +1/+0 and
//!   have vigilance." — an anthem static with no triggered/activated form;
//!   GAP'd.
//! "Hunt for Heresy — {1}, {T}: Tap target creature an opponent controls.
//!   Investigate." — fully modeled (tap the target, then create a Clue).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inquisitor Greyfax");
    let human = reg.interner_mut().intern("Human");
    let inquisitor = reg.interner_mut().intern("Inquisitor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(inquisitor);
    // GAP: static "Other creatures you control get +1/+0 and have vigilance" — anthem
    // continuous static, no triggered/activated form available here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Tap target creature an opponent controls. Investigate.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_and_investigate,
            }),
    )
}

fn tap_and_investigate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        out.push(Effect::Tap { target: *id });
    }
    out.push(Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Clue,
        count: 1,
    });
    out
}
