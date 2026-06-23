//! The Companion of the Wilds — `{5}` 3/3 Legendary Creature — Beast Noble
//! (colorless).
//!
//! Oracle:
//!   Old Companion — Your starting deck contains only cards from WOE, WOC, and
//!   playtest cards. (Companion deck-building restriction.)
//!   When The Companion of the Wilds enters the battlefield, create a Food
//!   token, a 1/1 black Rat creature token with "This creature can't block,"
//!   and a Royal role token attached to a creature you control.
//!
//! The "Role token, Food" Scryfall keywords are mechanic markers (companion +
//! role-token), not KeywordAbility variants — emitted as `keywords: vec![]`.
//! The ETB mints the Food token (commodity) and the Rat token; the Royal role
//! token is GAP'd (role-token attach with stat/keyword grant is not
//! expressible).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Companion of the Wilds");
    let beast = reg.interner_mut().intern("Beast");
    let noble = reg.interner_mut().intern("Noble");
    // Pre-intern the Rat token subtype for resolve-time lookup.
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "Old Companion — Your starting deck contains only cards from WOE, WOC,
    // and playtest cards" — a companion deck-building restriction, not a
    // triggered/activated/static board ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_tokens,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat").unwrap_or_default();
    let mut rat_subtypes = SubtypeSet::default();
    rat_subtypes.0.insert(rat);
    // GAP: the Rat token's "This creature can't block" static is not expressible
    // on a TokenDefinition (no per-token static-ability primitive) — minting the
    // bare 1/1 black Rat. GAP: the "Royal role token attached to a creature you
    // control" is not expressible (role-token attach with stat/keyword grant has
    // no primitive).
    vec![Effect::Sequence(vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: arcana_core::effects::CommodityToken::Food,
            count: 1,
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: rat,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: rat_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ])]
}
