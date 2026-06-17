//! The Charitable Drafter — `{4}{U}{U}` 3/5 Legendary Human Wizard.
//! ETB: create a 1/1 white Dog token, then a booster-draft sequence (GAP).
//! Static: creatures you control have a mana ability (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Charitable Drafter");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_dog,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: static "Creatures you control have '{T}: Add one mana of any
        // color. Spend this mana only to cast a card that wasn't in your
        // starting deck.'" — a granted activated ability with a spend
        // restriction; not expressible.
    )
}

fn etb_make_dog(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dog = reg.interner().lookup("Dog").unwrap_or_default();
    // GAP: "Then each player shuffles their hand into their library. Open a
    // Magic booster pack, then ... draft it face up ..." — booster-draft
    // mechanic not modeled. Only the Dog token is created.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dog,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: {
                let mut s = SubtypeSet::default();
                s.0.insert(dog);
                s
            },
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
