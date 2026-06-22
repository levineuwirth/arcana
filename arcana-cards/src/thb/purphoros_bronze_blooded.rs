//! Purphoros, Bronze-Blooded — `{4}{R}` 7/6 Legendary Enchantment Creature — God.
//! Indestructible.
//! "As long as your devotion to red is less than five, Purphoros isn't a
//! creature." — a continuous static type-altering ability, GAP'd.
//! "Other creatures you control have haste." — a continuous static anthem, GAP'd.
//! "{2}{R}: You may put a red creature card or an artifact creature card from
//! your hand onto the battlefield. Sacrifice it at the beginning of the next
//! end step." — the put-from-hand is wired (creature filter; the red-OR-artifact
//! restriction is a fidelity gap), but the delayed "sacrifice it at the next end
//! step" cannot be scheduled because the put creature's id is not returned, GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Purphoros, Bronze-Blooded");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };
    // GAP: static "isn't a creature while devotion to red < 5" — continuous type-altering static.
    // GAP: static "Other creatures you control have haste" — continuous anthem static.
    // GAP: "Sacrifice it at the beginning of the next end step" — cannot schedule a
    //   delayed sacrifice on the just-put creature (its id is not surfaced).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{R}: You may put a red creature card or an artifact creature card from your hand onto the battlefield."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_creature_from_hand,
        }),
    )
}

fn put_creature_from_hand(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
