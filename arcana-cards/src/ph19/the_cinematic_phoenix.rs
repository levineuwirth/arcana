//! The Cinematic Phoenix — `{3}{B}{R}` 4/4 Legendary Phoenix with Flying, Haste.
//! "Flying, haste, protection from red"
//! "{1}, {T}: Return target legendary creature card from your graveyard to the
//!  battlefield."
//! "Tap six untapped creatures you control: Return The Cinematic Phoenix from
//!  your graveyard to the battlefield. If you tapped six legendary creatures this
//!  way, you win the game."
//!
//! GAP: Protection (from red) is not an available KeywordAbility variant; the
//! protection portion of the keyword line is omitted (Flying + Haste emitted).
//! GAP: "If you tapped six legendary creatures this way, you win the game" — the
//! conditional win and the legendary-creature cost discrimination are not
//! expressible; only the graveyard self-return is implemented for ability #2.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Cinematic Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Return target legendary creature card from your graveyard \
                       to the battlefield."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature()
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_legend,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap six untapped creatures you control: Return The Cinematic Phoenix \
                       from your graveyard to the battlefield."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(ObjectFilter::creature()),
                    tap_other_count: 6,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_from_graveyard,
            }),
    )
}

fn reanimate_legend(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you tapped six legendary creatures, you win the game" not expressible.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
