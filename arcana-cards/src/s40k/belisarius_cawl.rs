//! Belisarius Cawl — `{2}{W}{U}` 2/4 Legendary Artifact Creature — Human.
//!
//! Ultima Founding — {T}, Tap two untapped artifacts you control: Create a
//!   2/2 white Astartes Warrior creature token with vigilance.
//! Master of Machines — {T}, Tap X untapped creatures you control: Look at
//!   the top X cards of your library. You may reveal an artifact card from
//!   among them and put it into your hand. Put the rest on the bottom of
//!   your library in a random order. — GAP (variable-X tap cost).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP: "Master of Machines — {T}, Tap X untapped creatures you control: …" The
// cost taps a VARIABLE number X of creatures; ActivationCost.tap_other_count is
// a fixed count, and the dig depth (look at top X) then scales by that same X —
// neither the variable-X tap cost nor the X-scaled dig is expressible.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Belisarius Cawl");
    let human = reg.interner_mut().intern("Human");
    // Pre-intern the token's creature type so the resolver can look it up.
    let _astartes = reg.interner_mut().intern("Astartes Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Tap two untapped artifacts you control: Create a 2/2 white Astartes Warrior creature token with vigilance.".into(),
            cost: ActivationCost {
                tap: true,
                tap_other: Some(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())),
                tap_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_astartes,
        }),
    )
}

fn make_astartes(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let astartes = reg.interner().lookup("Astartes Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    let token = TokenDefinition {
        name: astartes,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
