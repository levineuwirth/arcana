//! Mageta the Lion — `{3}{W}{W}` 3/3 Legendary white Human Spellshaper.
//! "{2}{W}{W}, {T}, Discard two cards: Destroy all creatures except for Mageta.
//! Those creatures can't be regenerated."
//! GAP: "Discard two cards" as an activation cost requires discard_self which only
//! discards the card itself. "Can't be regenerated" is not an Effect modifier.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mageta the Lion");
    let human = reg.interner_mut().intern("Human");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}{W}, {T}, Discard two cards: Destroy all creatures except for Mageta.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}{W}").unwrap(),
                    tap: true,
                    // GAP: "Discard two cards" as activation cost is not expressible;
                    // discard_self only discards the creature card itself from hand
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: wrath_except_self,
            }),
    )
}

fn wrath_except_self(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let _filter = ObjectFilter::creature()
        .without_supertypes(arcana_core::types::SupertypeSet::new().with(arcana_core::types::SupertypeSet::LEGENDARY));
    // Destroy all creatures (except Mageta — GAP: can't exclude self by id from ForEach)
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let targets: Vec<_> = ids.into_iter().filter(|&id| id != ctx.source).collect();
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
