//! Keskit, the Flesh Sculptor — `{2}{B}` 1/3 Legendary Creature —
//! Phyrexian Human Artificer.
//! {T}, Sacrifice three other artifacts and/or creatures: Look at the top
//! three cards of your library. Put two of them into your hand and the
//! other into your graveyard.
//! Partner. (Partner is not a usable KeywordAbility variant — GAP.)

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keskit, the Flesh Sculptor");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    // GAP: keyword "Partner" — not a usable KeywordAbility variant (multiplayer
    // commander-pairing mechanic).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice three other artifacts and/or creatures: Look at the top three cards of your library. Put two of them into your hand and the other into your graveyard.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(
                    ObjectFilter::new().with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                ),
                sacrifice_other_count: 3,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: dig_three,
        }),
    )
}

fn dig_three(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // FIDELITY GAP: oracle puts TWO of the three cards into hand and the other
    // into the graveyard. DigTopN is single-take only, so this puts one into
    // hand and the rest into the graveyard — the closest expressible form.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: None,
        rest: DigRest::Graveyard,
    }]
}
