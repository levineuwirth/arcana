//! Iizuka the Ruthless — `{3}{R}{R}` 3/3 Legendary Human Samurai.
//! "Bushido 2. {2}{R}, Sacrifice a Samurai: Samurai creatures you control
//! gain double strike until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iizuka the Ruthless");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bushido(2)],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, Sacrifice a Samurai: Samurai creatures you control gain double strike until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        subtypes: Some(vec![samurai]),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: samurai_double_strike,
            }),
    )
}

fn samurai_double_strike(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Samurai")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::DoubleStrike,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
