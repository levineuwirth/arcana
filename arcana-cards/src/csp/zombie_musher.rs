//! Zombie Musher — `{3}{B}` 2/3 Snow Zombie.
//! Snow swampwalk (snow landwalk — not the specific basic-land-type form);
//! `{S}: Regenerate this creature.`
//!
//! Snow landwalk is the generic/nonbasic-landwalk umbrella — NOT a basic-land-type
//! `<type>walk` — so it is not expressible as `KeywordAbility::Landwalk(...)` and is
//! emitted with no keyword (GAP). The regenerate activation is wired; `{S}` (snow
//! mana) is parsed as a snow pip in the mana cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zombie Musher");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Snow landwalk is generic/nonbasic landwalk, not a basic <type>walk.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{S}: Regenerate this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{S}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regen_self,
            }),
    )
}

fn regen_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
