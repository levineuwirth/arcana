//! Eladamri, Korvecdal — `{1}{G}{G}` 3/3 Legendary Elf Warrior.
//! You may look at the top card of your library any time. (static — GAP)
//! You may cast creature spells from the top of your library. (static — GAP)
//! `{G}, {T}, Tap two untapped creatures you control: Reveal a card from your
//! hand or the top card of your library. If you reveal a creature card this way,
//! put it onto the battlefield. Activate only during your turn.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eladamri, Korvecdal");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    // GAP: "You may look at the top card of your library any time" and "You may
    // cast creature spells from the top of your library" are continuous static
    // permission abilities not expressible as triggered/activated abilities.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}, {T}, Tap two untapped creatures you control: Reveal a card from your hand or the top card of your library. If you reveal a creature card this way, put it onto the battlefield. Activate only during your turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                tap: true,
                tap_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                tap_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: reveal_put_creature,
        }),
    )
}

fn reveal_put_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "reveal from hand OR top of library" choice isn't expressible as a
    // single effect, and the "activate only during your turn" timing window has
    // no field; model the put-creature-from-hand branch of the reveal.
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
