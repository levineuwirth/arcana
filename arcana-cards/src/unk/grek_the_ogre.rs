//! Grek the Ogre — `{3}{G}{G}` 4/4 Legendary Ogre.
//! Grek gets +1/+1 for each differently named token among permanents you
//! control. (GAP: dynamic static P/T with no matching helper.)
//! Whenever Grek attacks, create one of the following at random you don't
//! already control: a 1/1 green Donkey, a 2/2 white Cat, a 3/3 green Ogre
//! Noble, a Swamp token, a Food token, or a Gold token. (GAP: random choice
//! among heterogeneous tokens with a "don't already control" gate.)

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grek the Ogre");
    let ogre = reg.interner_mut().intern("Ogre");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — "Food" is not an available KeywordAbility.
        ..Default::default()
    };

    // GAP: static — "Grek gets +1/+1 for each differently named token among
    // permanents you control" is a dynamic continuous P/T with no expressible
    // helper.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: random_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn random_token(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create one of the following at random you don't already control"
    // requires a random selection over heterogeneous tokens gated by what you
    // already control; no Effect expresses this.
    Vec::new()
}
