//! Deathless Behemoth — `{6}` 6/6 Eldrazi. Vigilance.
//! "Sacrifice two Eldrazi Scions: Return this card from your graveyard to your
//! hand. Activate only as a sorcery." — graveyard-activated.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathless Behemoth");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let scion = reg.interner_mut().intern("Scion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let sac_filter = ObjectFilter::creature()
        .with_subtype_sym(eldrazi)
        .with_subtype_sym(scion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice two Eldrazi Scions: Return this card from your graveyard to your hand. Activate only as a sorcery.".into(),
            // GAP: "two Eldrazi Scions" — there is no sacrifice_other_count field;
            // modeled as sacrificing one Eldrazi Scion.
            cost: ActivationCost {
                sacrifice_other: Some(sac_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_to_hand,
        }),
    )
}

fn return_self_to_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
