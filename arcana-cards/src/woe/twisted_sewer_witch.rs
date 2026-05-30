//! Twisted Sewer-Witch — `{3}{B}{B}` 3/4 black Human Warlock.
//! "When this creature enters, create a 1/1 black Rat creature token with
//! 'This creature can't block.' Then for each Rat you control, create a
//! Wicked Role token attached to that Rat."
//!
//! GAP: Wicked Role token creation (attaching an Aura Role token to each Rat
//! you control) is not expressible with the current Effect catalog — Role
//! token attachment/creation is not a supported Effect variant.
//! The Rat token creation is implemented; the Role token loop is GAP'd.
//!
//! Note: "This creature can't block" on the Rat token is a static ability
//! on the token; TokenDefinition.abilities does not support static abilities,
//! so it is omitted (GAP: token static ability "can't block").

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twisted Sewer-Witch");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_rat_and_roles,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_rat_and_roles(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat")
        .expect("Rat interned during register()");
    let mut rat_subtypes = SubtypeSet::default();
    rat_subtypes.0.insert(rat);
    let rat_token = TokenDefinition {
        name: rat,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: rat_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: token static ability "This creature can't block" not expressible
        // via TokenDefinition.abilities
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: rat_token },
        // GAP: for each Rat you control, create a Wicked Role token attached
        // to that Rat — Role token creation/attachment is not a supported
        // Effect variant.
    ]
}
