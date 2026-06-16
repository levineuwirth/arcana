//! Wrenn and Realmbreaker — `{1}{G}{G}` Legendary Planeswalker — Wrenn, loyalty 5.
//!
//! Static: "Lands you control have \"{T}: Add one mana of any color.\"" — an
//!   ability-granting static (not a loyalty ability); not expressible from the
//!   demonstrated surface. GAP (static, not modeled).
//! +1: Up to one target land you control becomes a 3/3 Elemental creature with
//!     vigilance, hexproof, and haste until your next turn. It's still a land. The
//!     becomes-a-creature animation with base P/T + keywords until-your-next-turn is
//!     not expressible here — GAP body, shell kept with correct cost.
//! −2: Mill three cards. You may put a permanent card from among the milled cards into
//!     your hand. Mill three is modeled; the "may put a permanent into hand" rider
//!     keyed on the milled cards is not expressible — GAP that rider.
//! −7: You get an emblem with "You may play lands and cast permanent spells from your
//!     graveyard." Rule-altering casting/playing permission; not expressible by
//!     anthem/keyword/standard triggers. Emblem shell created with the grant GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and Realmbreaker");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        // GAP: keyword "Mill" — no usable KeywordAbility variant; the only Mill on
        //      this card is the -2 loyalty ability, which is modeled below.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target land you control becomes a 3/3 \
                       Elemental creature with vigilance, hexproof, and haste until \
                       your next turn. It's still a land."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Mill three cards. You may put a permanent card from among \
                       the milled cards into your hand."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"You may play lands and cast \
                       permanent spells from your graveyard.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 3/3 Elemental creature with vigilance, hexproof, and haste
    //      until your next turn; still a land" — land-animation with base P/T,
    //      type-add, keyword grant, and until-your-next-turn duration is not
    //      expressible from the demonstrated surface.
    Vec::new()
}

fn minus_two_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may put a permanent card from among the milled cards into your hand" —
    //      the milled-card-conditional return rider is not expressible. Mill three
    //      modeled.
    vec![Effect::Mill { player: ctx.controller, count: 3 }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Wrenn and Realmbreaker").expect("name interned");
    // GAP: "You may play lands and cast permanent spells from your graveyard" is a
    //      rule-altering casting/playing permission, not expressible by
    //      anthem/keyword/standard triggers. Emblem shell created.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
