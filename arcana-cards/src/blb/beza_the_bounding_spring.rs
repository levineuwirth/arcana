//! Beza, the Bounding Spring — `{2}{W}{W}` 4/5 legendary white Elemental Elk.
//! "When Beza enters, create a Treasure token if an opponent controls more
//! lands than you. You gain 4 life if an opponent has more life than you.
//! Create two 1/1 blue Fish creature tokens if an opponent controls more
//! creatures than you. Draw a card if an opponent has more cards in hand
//! than you."
//! GAP: keyword — Treasure token creation not a keyword in the supported set.
//! GAP: effect — conditional checks ("if an opponent controls more X than
//! you") are not expressible; emitting all four effects unconditionally as
//! best-effort.

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
    let name = reg.interner_mut().intern("Beza, the Bounding Spring");
    let elemental = reg.interner_mut().intern("Elemental");
    let elk = reg.interner_mut().intern("Elk");
    let _fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(elk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: all four effects are conditional on opponent comparisons; emitting
    // unconditionally as best-effort. Treasure token creation not in catalog.
    let fish = reg.interner().lookup("Fish").expect("Fish interned during register()");
    let mut fish_subtypes = SubtypeSet::default();
    fish_subtypes.0.insert(fish);
    let fish_token = TokenDefinition {
        name: fish,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: fish_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let fish_token2 = TokenDefinition {
        name: fish,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: {
            let mut s = SubtypeSet::default();
            s.0.insert(fish);
            s
        },
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::GainLife { player: trig.controller, amount: 4 },
        Effect::CreateToken { controller: trig.controller, token: fish_token },
        Effect::CreateToken { controller: trig.controller, token: fish_token2 },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ]
}
