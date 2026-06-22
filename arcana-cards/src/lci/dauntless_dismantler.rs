//! Dauntless Dismantler — `{1}{W}` 1/4 Human Artificer.
//! "{X}{X}{W}, Sacrifice this creature: Destroy each artifact with mana
//! value X."
//!
//! GAP (static): "Artifacts your opponents control enter tapped." — an
//! enters-tapped replacement static; no expressible triggered/activated
//! primitive.
//!
//! The activated ability is wired: cost {X}{X}{W} + sacrifice self; the
//! resolver reads X from ctx.x_value and destroys every artifact whose
//! mana value equals X via ForEach over a CMC-exact filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauntless Dismantler");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}{X}{W}, Sacrifice this creature: Destroy each artifact with mana value X.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}{X}{W}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: destroy_artifacts_with_cmc_x,
        }),
    )
}

fn destroy_artifacts_with_cmc_x(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .with_exact_cmc(x);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
